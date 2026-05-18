//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1244/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1244<F: Float>(t1045: F, t19836: F, t3117: F, t11866: F, t11927: F, t15716: F, t15771: F, t15774: F, t15776: F, t15817: F, t1671: F, t19819: F, t19827: F, t19831: F, t3115: F, t4831: F, t4834: F, t4869: F, t4879: F, t6273: F) -> F {
    let t19837 = t19836 * t1045;
    let t19838 = t3117 * t19837;
    let t19841 = -t15771 - t15774 + F::new(0.31758531939310916275e-3) * t15776 + F::new(0.28582678745379824648e-3) * t4834 * t4831 - F::new(0.12862205435420921092e-2) * t15716 * t19819 + F::new(0.42874018118069736972e-3) * t15817 * t1671 + F::new(0.42874018118069736972e-3) * t4879 * t4869 - F::new(0.14291339372689912324e-3) * t19827 + F::new(0.42874018118069736972e-3) * t11927 * t19831 - F::new(0.42874018118069736972e-3) * t11866 * t6273 - F::new(0.42874018118069736972e-3) * t3115 * t19838;
    t19841
}
