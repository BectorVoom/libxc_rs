//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 975/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk975<F: Float>(t19763: F, t2059: F, t3777: F, t3484: F, t3482: F, t3786: F, t3797: F, t14265: F, t14199: F, t6234: F, t2266: F, t3579: F, t13387: F, t13389: F, t13400: F, t19718: F, t19723: F, t19727: F, t19731: F, t19735: F, t19738: F, t19744: F, t19748: F, t19750: F, t19755: F, t19757: F, t19760: F, t19762: F) -> (F, F, F, F, F, F, F, F) {
    let t19765 = t19763 * t2059 * t3777;
    let t19766 = t3484 * t19765;
    let t19767 = t3482 * t19766;
    let t19770 = t3797 * t2059 * t3786;
    let t19771 = t14265 * t19770;
    let t19772 = t3482 * t19771;
    let t19774 = t14199 * t6234;
    let t19775 = t3482 * t19774;
    let t19777 = t2266 * t3579;
    let t19778 = t3484 * t19777;
    let t19779 = t3482 * t19778;
    let t19783 = -0.24872916666666666666e-2 * t19718 + 0.66327777777777777776e-2 * t19723 - 0.55273148148148148146e-2 * t19727 - 0.17687407407407407407e-1 * t19731 - 0.55273148148148148147e-2 * t19735 + t19738 + 0.44218518518518518517e-2 * t19744 - 0.13265555555555555555e-1 * t19748 - 0.66327777777777777776e-2 * t19750 - 0.66327777777777777776e-2 * t19755 - 0.58958024691358024689e-2 * t19757 - t19760 + t19762 - 0.3684876543209876543e-3 * t19767 + 0.33163888888888888888e-2 * t19772 + 0.88437037037037037034e-2 * t19775 - 0.22109259259259259258e-2 * t19779 + 0.33163888888888888888e-2 * t13387 + 0.88437037037037037034e-2 * t13389 + t13400;
    (t19765, t19767, t19770, t19772, t19775, t19777, t19779, t19783)
}
