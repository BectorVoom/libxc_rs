//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 768/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk768(t15488: f64, t15710: f64, t1139: f64, t3435: f64, t1136: f64, t3441: f64, t1138: f64, t285: f64, t1147: f64, t3443: f64, t3460: f64, t12652: f64, t12654: f64, t12656: f64, t12660: f64, t12665: f64, t12667: f64, t12669: f64, t12672: f64, t12675: f64, t12678: f64, t12683: f64, t12685: f64, t12687: f64, t12690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15711 = t15488 + t15710;
    let t15713 = t3435 * t1139;
    let t15716 = t1136 * t3441;
    let t15721 = t1138 * t1138;
    let t15722 = 1.0_f64 / t15721;
    let t15723 = t285 * t15722;
    let t15724 = t3443 * t1147;
    let t15727 = t1147 * t3460;
    let t15744 = 0.1875e0_f64 * t12652 - 0.45e1_f64 * t12654 - 0.1125e1_f64 * t12656 + 0.1125e1_f64 * t12660 - 0.2428125e0_f64 * t12665 + 0.485625e0_f64 * t12667 - 0.2428125e1_f64 * t12669 - 0.2428125e0_f64 * t12672 - 0.5625e0_f64 * t12675 - 0.225e1_f64 * t12678 + 0.485625e1_f64 * t12683 + 0.225e1_f64 * t12685 + 0.3375e1_f64 * t12687 + 0.2428125e1_f64 * t12690;
    (t15711, t15713, t15716, t15723, t15724, t15727, t15744)
}
