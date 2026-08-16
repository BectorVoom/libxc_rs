//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 969/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk969(t1991: f64, t2035: f64, t39: f64, t2071: f64, t554: f64, t138: f64, t38195: f64, t538: f64, t2001: f64, t2038: f64, t23810: f64, t23831: f64, t23869: f64, t37685: f64, t39839: f64, t39843: f64, t399: f64, t40055: f64, t40059: f64, t40078: f64, t543: f64, t8833: f64, t8852: f64, t8869: f64, t9003: f64) -> f64 {
    let t40178 = t1991 * t39 * t2035;
    let t40181 = t554 * t2071;
    let t40186 = t138 * t538 * t38195;
    let t40193 = -0.14498192132169191472e2_f64 * t9003 * t399 - 0.22187521816247557116e3_f64 * t23810 * t39839 + 0.11093760908123778558e3_f64 * t8852 * t39843 - 0.14498192132169191472e2_f64 * t23869 * t40059 + 0.14498192132169191472e2_f64 * t8833 * t40055 + 0.91821883503738212655e2_f64 * t23869 * t40078 + 0.87582322958871935983e1_f64 * t40178 * t2038 + 48.0_f64 * t2001 * t8869 * t40181 - 0.2607118765118496554e1_f64 * t23831 * t40186 + 0.86903958837283218463e0_f64 * t2001 * t40186 + 0.23380572188451859703e3_f64 * t543 * t37685;
    t40193
}
