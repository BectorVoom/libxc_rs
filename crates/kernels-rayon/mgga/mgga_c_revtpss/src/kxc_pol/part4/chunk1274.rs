//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1274/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1274(t3151: f64, t3154: f64, t15907: f64, t3117: f64, t11795: f64, t11859: f64, t11866: f64, t11875: f64, t15859: f64, t15862: f64, t15865: f64, t15866: f64, t15888: f64, t15892: f64, t15895: f64, t15899: f64, t15906: f64, t3184: f64, t375: f64, t4834: f64, t4912: f64) -> f64 {
    let t15908 = t3154 * t3151;
    let t15909 = t15907 * t15908;
    let t15910 = t3117 * t15909;
    let t15913 = 0.23818898954483187207e-3_f64 * t4834 * t3184 - 0.28582678745379824648e-3_f64 * t11795 + 0.72409452821628889107e-2_f64 * t15859 * t375 - 0.47637797908966374413e-4_f64 * t15862 + t15865 - 0.22866142996303859718e-2_f64 * t15866 * t375 + 0.21437009059034868486e-3_f64 * t15888 * t375 - t15892 - 0.42874018118069736972e-3_f64 * t11859 * t15895 + 0.21437009059034868486e-3_f64 * t11875 * t15899 - 0.42874018118069736972e-3_f64 * t11866 * t4912 - 0.12862205435420921092e-2_f64 * t15906 * t15910;
    t15913
}
