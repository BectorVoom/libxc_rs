//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 969/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk969<F: Float>(t1991: F, t2035: F, t39: F, t2071: F, t554: F, t138: F, t38195: F, t538: F, t2001: F, t2038: F, t23810: F, t23831: F, t23869: F, t37685: F, t39839: F, t39843: F, t399: F, t40055: F, t40059: F, t40078: F, t543: F, t8833: F, t8852: F, t8869: F, t9003: F) -> F {
    let t40178 = t1991 * t39 * t2035;
    let t40181 = t554 * t2071;
    let t40186 = t138 * t538 * t38195;
    let t40193 = -F::cast_from(0.14498192132169191472e2_f64) * t9003 * t399 - F::cast_from(0.22187521816247557116e3_f64) * t23810 * t39839 + F::cast_from(0.11093760908123778558e3_f64) * t8852 * t39843 - F::cast_from(0.14498192132169191472e2_f64) * t23869 * t40059 + F::cast_from(0.14498192132169191472e2_f64) * t8833 * t40055 + F::cast_from(0.91821883503738212655e2_f64) * t23869 * t40078 + F::cast_from(0.87582322958871935983e1_f64) * t40178 * t2038 + F::cast_from(48.0_f64) * t2001 * t8869 * t40181 - F::cast_from(0.2607118765118496554e1_f64) * t23831 * t40186 + F::cast_from(0.86903958837283218463e0_f64) * t2001 * t40186 + F::cast_from(0.23380572188451859703e3_f64) * t543 * t37685;
    t40193
}
