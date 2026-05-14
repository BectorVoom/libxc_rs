//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1170/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1170<F: Float>(t1593: F, t5546: F, t37977: F, t6426: F, t6449: F, t93164: F, t100558: F, t100688: F, t100697: F, t100698: F, t100706: F, t100708: F, t100725: F, t100734: F, t11321: F, t1669: F, t22522: F, t22552: F, t22568: F, t22613: F, t22696: F, t22755: F, t22767: F, t22852: F, t25643: F, t25644: F, t25704: F, t25734: F, t25839: F, t5536: F, t5569: F, t5579: F, t5611: F, t58524: F, t6438: F, t6446: F, t65750: F, t72: F, t73: F, t7837: F, t92425: F, t92579: F, t92629: F, t92654: F, t92920: F, t93163: F) -> (F, F, F) {
    let t100737 = t5546 * t1593;
    let t100741 = t6426 * t37977;
    let t100745 = t93164 * t6449;
    let t100748 = -0.89080607335887169332e-4 * t5569 * t73 * t100688 - 0.40859909362962962964e0 * t22552 * t22767 * t25839 + t100697 + 0.45967398033333333332e0 * t92579 * t5579 * t72 * t100698 + 0.76612330055555555556e-1 * t92425 * t6446 - 0.85124811172839506173e-2 * t100706 - 0.12768721675925925926e-1 * t5611 * t100708 + 0.44455354858818847408e-2 * t22852 * t6438 - 0.4945510644553639738e-5 * t92629 - 0.98978452595430188148e-4 * t92654 - 0.23238868087529279928e-3 * t25734 * t11321 + 8.0 * t22696 * t25644 + 8.0 * t1669 * t92920 * t25643 + 8.0 * t1669 * t22755 * t58524 + 4.0 * t1669 * t22755 * t100725 - 0.68099848938271604939e-1 * t22522 * t22568 * t25704 + t100734 - 0.6887280848989300204e-5 * t7837 * t5536 * t65750 * t100737 * t100558 + 0.44540303667943584666e-3 * t22613 * t73 * t100741 - 0.17263005832038132093e-5 * t93163 * t100745;
    (t100737, t100741, t100748)
}
