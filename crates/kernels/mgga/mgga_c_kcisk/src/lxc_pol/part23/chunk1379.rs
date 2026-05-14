//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1379/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1379<F: Float>(t113576: F, t9426: F, t1308: F, t388: F, t52538: F, t1333: F, t33577: F, t33541: F, t3748: F, t109891: F, t110219: F, t110304: F, t110308: F, t110384: F, t110423: F, t113861: F, t1220: F, t20: F, t2059: F, t20604: F, t2718: F, t32035: F, t32087: F, t32088: F, t33424: F, t33428: F, t33446: F, t3937: F, t394: F, t3988: F, t9796: F) -> (F, F, F) {
    let t114405 = 0.26805555555555555556e-2 * t9426 * t113576;
    let t114407 = t52538 * t388 * t1308;
    let t114437 = t1333 * t33577;
    let t114438 = 0.33163888888888888888e-2 * t114437;
    let t114439 = t3748 * t33541;
    let t114440 = 0.14739506172839506172e-2 * t114439;
    let t114441 = t114405 - 0.23280625000000000001e-2 * t114407 * t32035 + 0.40208333333333333335e-2 * t110304 * t9796 + 0.8041666666666666667e-2 * t110308 * t9796 - 0.10416666666666666667e-1 * t1220 * t20604 * t394 * t20 * t2718 + 0.69444444444444444446e-2 * t32087 * t113861 + 0.69444444444444444446e-2 * t110423 * t33424 + 0.69444444444444444446e-2 * t110384 * t33424 - 0.71481481481481481484e-2 * t110219 * t33428 + 0.69444444444444444446e-2 * t110423 * t33446 + 0.69444444444444444446e-2 * t110384 * t33446 + 0.34722222222222222223e-2 * t32087 * t3937 * t32088 * t2059 * t3988 + 0.16581944444444444444e-2 * t109891 - t114438 + t114440;
    (t114437, t114439, t114441)
}
