//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1232/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1232<F: Float>(t6589: F, t68: F, t13151: F, t1891: F, t225: F, t228: F, t230: F, t2379: F, t2553: F, t2667: F, t2671: F, t2672: F, t2675: F, t40848: F, t40972: F, t40977: F, t41241: F, t41242: F, t41244: F, t41245: F, t41248: F, t41249: F, t41263: F, t41297: F, t4225: F, t822: F, t824: F, t825: F, t9516: F, t9938: F, t9947: F, t9950: F, t9951: F, t9954: F) -> F {
    let t41315 = t68 * t6589;
    let t41332 = -(t41241 + t41242 + t41244 + t41245 + t41248 + t41249 + t41263 + t41297) * t225 * t230 + F::cast_from(12.0_f64) * t9938 * t825 - F::cast_from(72.0_f64) * t2667 * t2672 + F::cast_from(18.0_f64) * t2667 * t2675 + F::cast_from(240.0_f64) * t822 * t9947 - F::cast_from(144.0_f64) * t13151 * t9951 + F::cast_from(12.0_f64) * t822 * t9954 - F::cast_from(360.0_f64) * t228 * t41315 * t40972 + F::cast_from(360.0_f64) * t4225 * t1891 * t2379 * t2553 - F::cast_from(36.0_f64) * t228 * t2671 * t40977 - F::cast_from(48.0_f64) * t4225 * t9950 * t9516 + F::cast_from(3.0_f64) * t228 * t824 * t40848;
    t41332
}
