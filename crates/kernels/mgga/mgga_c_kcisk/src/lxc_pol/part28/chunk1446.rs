//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1446/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1446<F: Float>(t10028: F, t122139: F, t122141: F, t122143: F, t122146: F, t122148: F, t122150: F, t122152: F, t122446: F, t122470: F, t122503: F, t122530: F, t122551: F, t122594: F, t122616: F, t122646: F, t122684: F, t122713: F, t122730: F, t122771: F, t122799: F, t122830: F, t122855: F, t122872: F, t122902: F, t122929: F, t122973: F, t122995: F, t123021: F, t123044: F, t123082: F, t123114: F, t123140: F, t123158: F, t123189: F, t123207: F, t123231: F, t123256: F, t123285: F, t123312: F, t12345: F, t12352: F, t18925: F, t25166: F, t25170: F, t2666: F, t33153: F, t33306: F, t34615: F, t34650: F, t35344: F, t5532: F, t64998: F, t71472: F, t802: F, t9262: F, t9291: F, t9763: F, t9772: F) -> (F,) {
    let t123328 = 4.0 * t33153 * t25166 - t122139 + 2.0 * t33153 * t25170 + 4.0 * t18925 * t34615 + t122141 - t33306 * t9291 - 6.0 * t12352 * t9772 * t9262 + t122143 + t122146 + 2.0 * t5532 * t9772 * t9291 - t122148 + (t122446 + t122470 + t122503 + t123231 + t123256 + t123312 + t123082 + t122929 + t122973 + t123189 + t122902 + t122872 + t122995 + t123158 + t123140 + t123114 + t123285 + t123207 + t122855 + t122830 + t123044 + t122799 + t123021 + t122771 + t122730 + t122713 + t122684 + t122646 + t122616 + t122594 + t122551 + t122530) * t802 + 4.0 * t64998 * t10028 + t122150 + 2.0 * t12345 * t35344 + 2.0 * t71472 * t9763 - t122152 + 4.0 * t5532 * t34650 * t2666;
    (t123328,)
}
