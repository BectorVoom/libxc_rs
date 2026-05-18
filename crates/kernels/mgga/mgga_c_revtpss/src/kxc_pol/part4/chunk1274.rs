//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1274/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1274<F: Float>(t3151: F, t3154: F, t15907: F, t3117: F, t11795: F, t11859: F, t11866: F, t11875: F, t15859: F, t15862: F, t15865: F, t15866: F, t15888: F, t15892: F, t15895: F, t15899: F, t15906: F, t3184: F, t375: F, t4834: F, t4912: F) -> F {
    let t15908 = t3154 * t3151;
    let t15909 = t15907 * t15908;
    let t15910 = t3117 * t15909;
    let t15913 = F::new(0.23818898954483187207e-3) * t4834 * t3184 - F::new(0.28582678745379824648e-3) * t11795 + F::new(0.72409452821628889107e-2) * t15859 * t375 - F::new(0.47637797908966374413e-4) * t15862 + t15865 - F::new(0.22866142996303859718e-2) * t15866 * t375 + F::new(0.21437009059034868486e-3) * t15888 * t375 - t15892 - F::new(0.42874018118069736972e-3) * t11859 * t15895 + F::new(0.21437009059034868486e-3) * t11875 * t15899 - F::new(0.42874018118069736972e-3) * t11866 * t4912 - F::new(0.12862205435420921092e-2) * t15906 * t15910;
    t15913
}
