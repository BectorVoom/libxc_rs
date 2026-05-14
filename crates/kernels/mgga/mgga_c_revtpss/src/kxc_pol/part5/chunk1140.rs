//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1140/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1140<F: Float>(t19456: F, t247: F, t3116: F, t3172: F, t6311: F, t3161: F, t1043: F, t6244: F, t1045: F, t3117: F, t1668: F, t4772: F, t11866: F, t11927: F, t15716: F, t15771: F, t15774: F, t15776: F, t15817: F, t1671: F, t3115: F, t4831: F, t4834: F, t4869: F, t4879: F, t6273: F) -> (F, F, F) {
    let t19819 = t247 * t3116 * t19456;
    let t19826 = t3172 * t6311;
    let t19827 = t3161 * t19826;
    let t19829 = t6244 * t1043;
    let t19830 = t19829 * t1045;
    let t19831 = t3117 * t19830;
    let t19836 = t4772 * t1668;
    let t19837 = t19836 * t1045;
    let t19838 = t3117 * t19837;
    let t19841 = -t15771 - t15774 + 0.31758531939310916275e-3 * t15776 + 0.28582678745379824648e-3 * t4834 * t4831 - 0.12862205435420921092e-2 * t15716 * t19819 + 0.42874018118069736972e-3 * t15817 * t1671 + 0.42874018118069736972e-3 * t4879 * t4869 - 0.14291339372689912324e-3 * t19827 + 0.42874018118069736972e-3 * t11927 * t19831 - 0.42874018118069736972e-3 * t11866 * t6273 - 0.42874018118069736972e-3 * t3115 * t19838;
    (t19829, t19836, t19841)
}
