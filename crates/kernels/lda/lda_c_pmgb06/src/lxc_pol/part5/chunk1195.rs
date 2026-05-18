//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1195/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1195<F: Float>(t10577: F, t11206: F, t11237: F, t11305: F, t11311: F, t11406: F, t11407: F, t11465: F, t11640: F, t1316: F, t14875: F, t18474: F, t18582: F, t18609: F, t18622: F, t18796: F, t18807: F, t18903: F, t18926: F, t21317: F, t21358: F, t21361: F, t21366: F, t21367: F, t21369: F, t21397: F, t21399: F, t21406: F, t21409: F, t21411: F, t21414: F, t21416: F, t21452: F, t21468: F, t21501: F, t21506: F, t21599: F, t2209: F, t2241: F, t2255: F, t2722: F, t2730: F, t2733: F, t346: F, t370: F, t374: F, t384: F, t387: F, t4232: F, t5583: F, t5831: F, t5834: F, t6006: F, t6018: F, t63: F, t7043: F, t7053: F, t7056: F, t7060: F, t73: F, t74: F, t783: F, t787: F, t7881: F, t790: F, t7920: F, t7921: F, t8081: F, t8087: F, t8091: F, t8094: F, t8097: F, t8099: F, t8105: F, t8266: F, t8282: F, t8285: F, t8287: F, t8291: F, t8293: F, t8295: F, t8339: F, t8346: F, t8353: F, t8358: F, t8388: F) -> F {
    let t21619 = t8081 - t8087 - t8091 - F::new(0.15965645347006147) * t8094 - t8097 + t8099 + t8105 + F::new(9.0) * t1316 * t790 * t14875 + F::new(9.0) * t1316 * t790 * t18903 + F::new(2.0) * t346 * t2733 * t2255 + t346 * ((-F::new(18.0) * t11407 * t21411 - F::new(1.46904) * t63 * t370 * t21358 + t11305 - F::new(0.97936) * t8282 + t8346 + t8291 + t21501 - t21414 + t21416 + t8293 - t8295 - F::new(8.81424) * t18582 + t21406 - t11311 + t21399 - t21369 + F::new(1.95872) * t8388 + F::new(6.0) * t18622 - t8339 + F::new(14.0) / F::new(27.0) * t8358 + t21317 - F::new(2.93808) * t11237 - t21361 + t21366 - t21367 - t11406 - t8266 - t21409 + F::new(2.2851733333333333) * t8353 + t21452 + t8285 + t8287 + t21397 - F::new(3.0) / F::new(2.0) * t18609 + t21468) * t74 - t21506 * t384 - F::new(3.0) * t18796 * t787 + F::new(6.0) * t18807 * t2241 - F::new(3.0) * t7043 * t2255 + F::new(6.0) * t11206 * t2722 - F::new(18.0) * t11465 * t7053 + F::new(12.0) * t5834 * t7056 - F::new(3.0) * t5831 * t2730 + F::new(6.0) * t5834 * t7060 + t21599) * t387 * t73 - F::new(6.0) * t6006 * t18926 * t7881 * t374 - F::new(9.0) * t11640 * t7921 + F::new(18.0) * t6018 * t18474 - F::new(9.0) * t5583 * t10577 * t7920 - F::new(9.0) * t5583 * t4232 * t783 * t2209;
    t21619
}
