//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1127/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1127<F: Float>(t25752: F, t29482: F, t25838: F, t930: F, t25643: F, t22563: F, t4441: F, t7983: F, t4466: F, t5537: F, t5546: F, t100580: F, t100678: F, t100697: F, t100706: F, t101193: F, t11121: F, t115405: F, t115506: F, t1557: F, t15879: F, t1603: F, t22522: F, t22619: F, t22718: F, t22834: F, t25713: F, t29474: F, t29531: F, t3188: F, t5538: F, t5540: F, t5579: F, t5598: F, t72: F, t73: F, t73772: F, t92358: F, t92471: F, t92654: F, t93047: F, t93048: F, t93165: F, t938: F) -> (F, F, F, F) {
    let t115650 = t29482 * t25752;
    let t115654 = t930 * t25838;
    let t115658 = t930 * t25643;
    let t115664 = t7983 * t22563 * t4441;
    let t115680 = t5537 * t5546 * t4466;
    let t115683 = 0.60548059007656442388e-3 * t93047 * t93048 * t100580 * t25713 + 0.34049924469135802469e-1 * t22522 * t100678 * t938 * t1557 * t3188 - 0.44540303667943584666e-3 * t22619 * t73 * t115506 - 0.17263005832038132092e-5 * t115650 * t93165 + t100697 - 0.17024962234567901235e-1 * t100706 - 0.17816121467177433866e-2 * t92471 * t92358 * t115654 + 0.267241822007661508e-2 * t101193 * t92358 * t115658 - 0.49489226297715094074e-4 * t92654 - 0.81118562704294997117e-4 * t73772 * t115664 + 0.46509801892875584e-2 * t22834 * t29531 + 0.46509801892875584e-2 * t1603 * t22718 * t29474 + 0.38306165027777777778e-1 * t5598 * t5579 * t72 * t15879 + 0.25845121844514357744e-4 * t5538 * t5540 * t115405 - 0.60102574844279699039e-6 * t11121 * t115680;
    (t115654, t115658, t115664, t115683)
}
