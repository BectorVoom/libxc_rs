//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1942/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1942<F: Float>(t1353: F, t22496: F, t13514: F, t93: F, t116: F, t28683: F, t2055: F, t2371: F, t1459: F, t1461: F, t1518: F, t18214: F, t1916: F, t1918: F, t2113: F, t2327: F, t26716: F, t26730: F, t26734: F, t26737: F, t28956: F, t28975: F, t28978: F, t28981: F, t28986: F, t4158: F, t4165: F, t572: F, t5795: F, t670: F, t7554: F, t7983: F, t8118: F, t8124: F, t8127: F) -> (F, F, F) {
    let t101479 = t22496 * t1353;
    let t101522 = t93 * t13514;
    let t101705 = t116 * t28683;
    let t101720 = t2371 * t2055;
    let t101724 = F::new(12.0) * t101705 * t572 * t670 + F::new(6.0) * t101720 * t1518 * t572 + F::new(6.0) * t2327 * t572 * t7983 + F::new(6.0) * t2371 * t28986 * t572 + F::new(12.0) * t1459 * t28975 + F::new(12.0) * t1459 * t28978 + F::new(12.0) * t1459 * t28981 + F::new(6.0) * t1461 * t28956 + F::new(3.0) * t18214 * t2113 + F::new(6.0) * t1916 * t26730 + F::new(12.0) * t1916 * t26734 + F::new(6.0) * t1916 * t26737 + F::new(3.0) * t1918 * t26716 + F::new(6.0) * t4158 * t8124 + F::new(3.0) * t4158 * t8127 + F::new(3.0) * t4165 * t8118 + F::new(12.0) * t5795 * t7554;
    (t101479, t101522, t101724)
}
