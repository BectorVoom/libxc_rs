//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1373/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1373<F: Float>(t109717: F, t1339: F, t6225: F, t32000: F, t3759: F, t5635: F, t110347: F, t110492: F, t110548: F, t110556: F, t114223: F, t114225: F, t114231: F, t114241: F, t114245: F, t114248: F, t32019: F, t33389: F, t9429: F, t9454: F, t9796: F) -> (F, F, F) {
    let t114251 = t1339 * t109717 * t6225;
    let t114254 = t3759 * t32000 * t5635;
    let t114258 = 0.66327777777777777776e-2 * t114223 - 0.21444444444444444446e-1 * t114225 * t9429 - 0.18518518518518518519e-1 * t110548 - 0.18518518518518518519e-1 * t110556 + 0.20833333333333333334e-1 * t114231 * t9454 + 0.20833333333333333334e-1 * t114231 * t9429 + 0.10416666666666666667e-1 * t110492 * t9796 + 0.10416666666666666667e-1 * t110347 * t9796 - 0.22109259259259259258e-2 * t114241 + 0.66327777777777777776e-2 * t114245 - 0.55273148148148148146e-2 * t114248 + 0.88437037037037037034e-2 * t114251 - 0.14739506172839506172e-1 * t114254 - 0.41666666666666666668e-1 * t32019 * t33389;
    (t114251, t114254, t114258)
}
