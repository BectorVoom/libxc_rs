//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 768/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk768<F: Float>(t15488: F, t15710: F, t1139: F, t3435: F, t1136: F, t3441: F, t1138: F, t285: F, t1147: F, t3443: F, t3460: F, t12652: F, t12654: F, t12656: F, t12660: F, t12665: F, t12667: F, t12669: F, t12672: F, t12675: F, t12678: F, t12683: F, t12685: F, t12687: F, t12690: F) -> (F, F, F, F, F, F, F) {
    let t15711 = t15488 + t15710;
    let t15713 = t3435 * t1139;
    let t15716 = t1136 * t3441;
    let t15721 = t1138 * t1138;
    let t15722 = F::new(1.0) / t15721;
    let t15723 = t285 * t15722;
    let t15724 = t3443 * t1147;
    let t15727 = t1147 * t3460;
    let t15744 = F::new(0.1875e0) * t12652 - F::new(0.45e1) * t12654 - F::new(0.1125e1) * t12656 + F::new(0.1125e1) * t12660 - F::new(0.2428125e0) * t12665 + F::new(0.485625e0) * t12667 - F::new(0.2428125e1) * t12669 - F::new(0.2428125e0) * t12672 - F::new(0.5625e0) * t12675 - F::new(0.225e1) * t12678 + F::new(0.485625e1) * t12683 + F::new(0.225e1) * t12685 + F::new(0.3375e1) * t12687 + F::new(0.2428125e1) * t12690;
    (t15711, t15713, t15716, t15723, t15724, t15727, t15744)
}
