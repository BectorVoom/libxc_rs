//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1136/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1136<F: Float>(t10560: F, t1775: F, t10261: F, t10388: F, t10575: F, t10580: F, t2: F, t2681: F, t2682: F, t2739: F, t42071: F, t42075: F, t42088: F, t42096: F, t43843: F, t43848: F, t43850: F, t43852: F, t43860: F, t43867: F, t43872: F, t43874: F, t462: F, t848: F) -> F {
    let t43879 = t1775 * t10560;
    let t43881 = -F::cast_from(8.0_f64) * t43843 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t462 * t10580 * t42088 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t43848 - F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t43850 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t462 * t43852 * t42096 + F::cast_from(8.0_f64) * t462 * t2681 * t10575 * t10388 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t43860 - F::cast_from(36.0_f64) * t462 * t10261 * t2 * t2682 * t2739 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t43867 + F::cast_from(8.0_f64) * t462 * t848 * t42071 + F::cast_from(112.0_f64) / F::cast_from(81.0_f64) * t43872 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t43874 + F::cast_from(2.0_f64) * t462 * t848 * t42075 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t43879;
    t43881
}
