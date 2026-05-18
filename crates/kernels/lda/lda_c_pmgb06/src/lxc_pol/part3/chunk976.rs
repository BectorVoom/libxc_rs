//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 976/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk976<F: Float>(t2209: F, t374: F, t4232: F, t1773: F, t2262: F, t2266: F, t26: F, t4405: F, t10541: F, t10565: F, t10625: F, t11188: F, t11200: F, t11206: F, t11211: F, t11222: F, t11225: F, t11229: F, t11230: F, t11231: F, t11236: F, t11237: F, t11289: F, t11299: F, t11305: F, t11308: F, t11311: F, t11313: F, t11318: F, t11320: F, t11323: F, t11366: F, t11370: F, t11374: F, t11377: F, t11380: F, t11382: F, t11385: F, t11388: F, t11391: F, t11393: F, t11396: F, t11428: F, t11439: F, t11451: F, t11465: F, t11493: F, t11511: F, t11524: F, t11525: F, t11558: F, t123: F, t1234: F, t1297: F, t1309: F, t1316: F, t2180: F, t2181: F, t2241: F, t2255: F, t2258: F, t2312: F, t315: F, t317: F, t342: F, t346: F, t35: F, t3559: F, t3588: F, t360: F, t3615: F, t3625: F, t3632: F, t3633: F, t3636: F, t378: F, t387: F, t4021: F, t4231: F, t4394: F, t5583: F, t5737: F, t5740: F, t5834: F, t5846: F, t5849: F, t63: F, t73: F, t74: F, t76: F, t769: F, t790: F, t791: F, t8061: F, t8245: F, t8293: F, t8295: F, t8399: F) -> F {
    let t11564 = t4232 * t2209 * t374;
    let t11567 = t1773 * t2262;
    let t11568 = F::new(0.15965645347006147) * t11567;
    let t11569 = t1773 * t2266;
    let t11574 = t4405 * t26;
    let t11582 = F::new(6.0) * t10625 * t791 + F::new(0.020267214298646783) * t123 * t315 * t11188 * t317 + F::new(18.0) * t2180 * t2258 * t4021 + F::new(18.0) * t2180 * t76 * t4394 * t342 + F::new(18.0) * t11200 * t5737 + F::new(6.0) * t2180 * t2181 * t3559 + t346 * (F::new(6.0) * t11206 * t1297 + (-t360 * t11385 / F::new(2.0) - F::new(88.1424) * t11230 * t11231 + t11318 + t11305 + t11380 + t11225 + t11229 + t11299 + F::new(176.2848) * t63 * t8245 * t769 * t3588 - F::new(88.1424) * t63 * t3615 * t2209 * t1234 - F::new(18.0) * t360 * t35 * t5740 * t1234 - t11222 - F::new(3.0) * t11313 + t8293 - t8295 - t11289 + F::new(2.20356) * t11370 - t11308 - t11311 + t11451 - t11236 - t11396 - F::new(0.97936) * t11237 + t11211 + F::new(2.2851733333333333) * t11388 + t11391 + F::new(14.0) / F::new(27.0) * t11393 + t11439 + t11366 + t11428 + F::new(6.0) * t11320 + t11323 + t11382 / F::new(2.0) - t11374 + t11377) * t74 - F::new(18.0) * t3632 * t2255 * t1297 + F::new(6.0) * t8399 * t2241 + F::new(12.0) * t3625 * t5846 + F::new(6.0) * t3625 * t5849 - F::new(6.0) * t11465 * t3633 + F::new(6.0) * t5834 * t3636 - t378 * (t11493 + t11511 + t11524 + t11525) - F::new(18.0) * t3632 * t2241 * t1309 + t11558) * t387 * t73 - F::new(9.0) * t4231 * t11564 - t11568 - F::new(0.15965645347006147) * t11569 + F::new(9.0) * t1316 * t790 * t10565 + F::new(18.0) * t11574 * t2312 + F::new(9.0) * t1316 * t790 * t8061 + F::new(18.0) * t5583 * t10541;
    t11582
}
