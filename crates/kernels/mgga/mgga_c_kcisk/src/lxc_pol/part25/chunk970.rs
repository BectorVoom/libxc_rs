//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 970/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk970<F: Float>(t10785: F, t10787: F, t10789: F, t10795: F, t10817: F, t17165: F, t17169: F, t17172: F, t17184: F, t17187: F, t17190: F, t17194: F, t17199: F, t17208: F, t1773: F, t1778: F, t2460: F, t4989: F, t5009: F, t5026: F, t7208: F, t7219: F, t7248: F, t7264: F) -> (F,) {
    let t17209 = -0.71963154864709268852e-1 * t4989 * t7248 + 0.83957014008827480328e-1 * t17165 + 0.17990788716177317213e-1 * t10817 * t2460 - 0.95950873152945691806e-1 * t17169 * t1778 - 0.31983624384315230602e-1 * t17172 - 0.47975436576472845903e-1 * t7219 * t5026 - 0.63967248768630461204e-1 * t7219 * t5009 + 0.17990788716177317213e-1 * t7208 * t5026 + 0.23987718288236422951e-1 * t7208 * t5009 + 0.71963154864709268853e-1 * t17184 + t17187 - 0.32383419689119170984e0 * t1773 * t17190 + 0.10794473229706390328e0 * t1773 * t17194 + 0.21588946459412780656e0 * t1773 * t17199 + 0.35981577432354634426e-1 * t10785 + 0.17990788716177317213e-1 * t10787 - 0.2398771828823642295e-1 * t10789 + t10795 + 0.21588946459412780656e0 * t4989 * t7264 + t17208;
    (t17209,)
}
