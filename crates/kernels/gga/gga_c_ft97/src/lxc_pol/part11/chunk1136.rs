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
    let t43881 = -F::new(8.0) * t43843 + F::new(40.0) / F::new(9.0) * t462 * t10580 * t42088 - F::new(8.0) / F::new(9.0) * t43848 - F::new(16.0) / F::new(27.0) * t43850 - F::new(80.0) / F::new(81.0) * t462 * t43852 * t42096 + F::new(8.0) * t462 * t2681 * t10575 * t10388 + F::new(40.0) / F::new(81.0) * t43860 - F::new(36.0) * t462 * t10261 * t2 * t2682 * t2739 + F::new(8.0) / F::new(3.0) * t43867 + F::new(8.0) * t462 * t848 * t42071 + F::new(112.0) / F::new(81.0) * t43872 + F::new(16.0) / F::new(9.0) * t43874 + F::new(2.0) * t462 * t848 * t42075 - F::new(16.0) / F::new(9.0) * t43879;
    t43881
}
