//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 810/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk810<F: Float>(t10845: F, t1268: F, t4965: F, t10850: F, t21181: F, t2917: F, t10838: F, t18877: F, t18900: F, t18902: F, t21839: F, t21843: F, t21847: F, t21852: F, t21856: F, t2265: F, t631: F) -> (F, F, F) {
    let t21863 = t10845 * t4965 * t1268;
    let t21867 = t2917 * t10850 * t21181;
    let t21870 = t631 * t21839 / F::cast_from(2.0_f64) - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t631 * t21843 + t631 * t21847 / F::cast_from(6.0_f64) + F::cast_from(6.0_f64) * t631 * t21852 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t631 * t21856 + t10838 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18877 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18900 - t18902 / F::cast_from(3.0_f64) - t2265 * t21863 / F::cast_from(3.0_f64) - t631 * t21867 / F::cast_from(3.0_f64);
    (t21863, t21867, t21870)
}
