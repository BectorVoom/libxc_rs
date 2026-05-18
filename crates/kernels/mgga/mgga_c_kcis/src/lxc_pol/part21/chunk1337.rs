//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1337/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1337<F: Float>(t95453: F, t95455: F, t95457: F, t95459: F, t95461: F, t95464: F, t95466: F, t95468: F, t95470: F, t95472: F, t95475: F, t11220: F, t11223: F, t11230: F, t1282: F, t1291: F, t15109: F, t15692: F, t15788: F, t2205: F, t27105: F, t27134: F, t28076: F, t28265: F, t28295: F, t34662: F, t3669: F, t3699: F, t5360: F, t5363: F, t5394: F, t7823: F, t8108: F, t8117: F, t92576: F, t95285: F, t95288: F, t95291: F, t95336: F, t95338: F, t95340: F, t95343: F, t95346: F, t95349: F, t95352: F, t95354: F, t95356: F, t95358: F, t95432: F, t95434: F, t95436: F, t95438: F, t95440: F, t95442: F, t95444: F, t95446: F, t95448: F, t95450: F, t95481: F, t96594: F, t96617: F, t96640: F) -> F {
    let t96663 = F::new(0.61111111111111111112e0) * t95453 - F::new(0.5625e0) * t95455 + F::new(0.89930555555555555557e-2) * t95457 - F::new(0.10791666666666666667e0) * t95459 - F::new(0.21583333333333333334e0) * t95461 + F::new(0.25e0) * t95464 + F::new(0.4046875e-1) * t95466 - F::new(0.20234375e-1) * t95468 - F::new(0.21583333333333333334e0) * t95470 + F::new(0.53958333333333333334e-1) * t95472 + F::new(0.41666666666666666666e-1) * t95475;
    let t96668 = -F::new(2.0) * t15109 * t7823 + F::new(2.0) * t3669 * t8117 * t3699 + F::new(4.0) * t3669 * t28295 * t1291 + F::new(2.0) * t34662 * t8108 - F::new(12.0) * t11230 * t28076 * t1291 + F::new(4.0) * t92576 * t5363 + F::new(4.0) * t11223 * t28076 - t5360 * t27134 + t95285 + t95288 + F::new(4.0) * t15692 * t27105 + F::new(4.0) * t3669 * t7823 * t5394 - t11220 * t8117 + t95291 - t28265 * t3699 + F::new(2.0) * t3669 * t2205 * t15788 - t1282 * (t96594 + F::new(0.53958333333333333334e-1) * t95336 - F::new(0.125e0) * t95338 + F::new(0.1875e0) * t95340 - F::new(0.809375e-1) * t95343 + F::new(0.95925925925925925927e-1) * t95346 - F::new(0.17986111111111111111e-1) * t95349 + F::new(0.1875e0) * t95352 + F::new(0.375e0) * t95354 + F::new(0.5e0) * t95356 + F::new(0.20833333333333333333e-1) * t95358 + t96617 + t96640 + F::new(0.20234375e-1) * t95432 - F::new(0.26979166666666666667e-1) * t95434 + F::new(0.625e-1) * t95436 + F::new(0.4046875e-1) * t95438 + F::new(0.26979166666666666667e-1) * t95440 + F::new(0.27777777777777777777e-1) * t95442 - F::new(0.89930555555555555557e-2) * t95444 + F::new(0.375e0) * t95446 - F::new(0.9375e-1) * t95448 + F::new(0.20234375e-1) * t95450 + t96663) + t95481;
    t96668
}
