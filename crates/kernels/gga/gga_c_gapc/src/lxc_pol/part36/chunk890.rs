//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 890/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk890<F: Float>(t11928: F, t11931: F, t11935: F, t11939: F, t11942: F, t11946: F, t11949: F, t11951: F, t11955: F, t11958: F, t11962: F, t11967: F, t11972: F, t11975: F, t11981: F, t11984: F, t11988: F, t11992: F, t11995: F, t11998: F) -> (F,) {
    let t12568 = 0.96681162811134562535e-8 * t11928 + 0.13505639832369200846e-5 * t11931 - 0.13493923611111111112e-4 * t11935 - 0.1374296967252737644e-5 * t11939 + 0.22509399720615334743e-7 * t11942 - 0.44197102999375800017e-8 * t11946 - 0.66295654499063700024e-7 * t11949 - 0.13259130899812740005e-6 * t11951 - 0.20240885416666666668e-3 * t11955 + 0.40481770833333333335e-4 * t11958 + 0.8433702256944444445e-6 * t11962 - 0.73794894748263888894e-6 * t11967 - 0.94685814672924837676e-4 * t11972 + 0.11594181388521408695e-4 * t11975 - 0.71696352428860134555e-9 * t11981 + 0.66295654499063700024e-7 * t11984 + 0.69504740211613770836e-3 * t11988 - 0.9275345110817126956e-4 * t11992 + 0.9275345110817126956e-4 * t11995 + 0.77294542590142724635e-6 * t11998;
    (t12568,)
}
