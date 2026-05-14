//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 804/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk804<F: Float>(t9775: F, t9777: F, t9780: F, t9783: F, t9789: F, t9791: F, t9793: F, t9796: F, t9800: F, t9802: F, t9805: F, t9808: F, t9811: F, t9816: F, t9818: F, t9820: F, t9822: F, t9824: F, t9828: F, t9830: F, t9833: F, t9836: F, t9839: F, t9847: F, t9850: F, t9853: F) -> (F, F) {
    let t10946 = 0.12328882118870421572e-6 * t9775 + 0.9275345110817126956e-4 * t9777 + 0.77294542590142724634e-6 * t9780 - 0.1374296967252737644e-5 * t9783 - 0.56273499301538336858e-8 * t9789 - 0.9275345110817126956e-4 * t9791 + 0.132681342766433194e-5 * t9793 - 0.55603792169291016668e-2 * t9796 - 0.29517957899305555558e-5 * t9800 - 0.2698425785107458272e-5 * t9802 - 0.15176747947735985782e-6 * t9805 + 0.2698425785107458272e-6 * t9808 - 0.57970906942607043472e-5 * t9811;
    let t10961 = -0.11594181388521408694e-4 * t9816 + 0.57970906942607043472e-5 * t9818 - 0.24326659074064819792e-2 * t9820 - 0.12974218172834570556e-1 * t9822 - 0.12974218172834570556e-1 * t9824 + 0.28985453471303521736e-5 * t9828 - 0.15458908518028544927e-5 * t9830 + 0.2748593934505475288e-5 * t9833 + 0.34752370105806885418e-3 * t9836 + 0.51491428373437201896e-5 * t9839 - 0.45839761994185933919e-8 * t9847 - 0.42270452978984302532e-6 * t9850 - 0.24760339692676868218e-5 * t9853;
    (t10946, t10961)
}
