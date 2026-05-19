//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1006/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1006<F: Float>(t11928: F, t11931: F, t11935: F, t11939: F, t11942: F, t11946: F, t11949: F, t11951: F, t11955: F, t11958: F, t11962: F, t11967: F, t11972: F, t11975: F, t11981: F, t11984: F, t11988: F, t11992: F, t11995: F, t11998: F) -> F {
    let t12568 = F::cast_from(0.96681162811134562535e-8_f64) * t11928 + F::cast_from(0.13505639832369200846e-5_f64) * t11931 - F::cast_from(0.13493923611111111112e-4_f64) * t11935 - F::cast_from(0.1374296967252737644e-5_f64) * t11939 + F::cast_from(0.22509399720615334743e-7_f64) * t11942 - F::cast_from(0.44197102999375800017e-8_f64) * t11946 - F::cast_from(0.66295654499063700024e-7_f64) * t11949 - F::cast_from(0.13259130899812740005e-6_f64) * t11951 - F::cast_from(0.20240885416666666668e-3_f64) * t11955 + F::cast_from(0.40481770833333333335e-4_f64) * t11958 + F::cast_from(0.8433702256944444445e-6_f64) * t11962 - F::cast_from(0.73794894748263888894e-6_f64) * t11967 - F::cast_from(0.94685814672924837676e-4_f64) * t11972 + F::cast_from(0.11594181388521408695e-4_f64) * t11975 - F::cast_from(0.71696352428860134555e-9_f64) * t11981 + F::cast_from(0.66295654499063700024e-7_f64) * t11984 + F::cast_from(0.69504740211613770836e-3_f64) * t11988 - F::cast_from(0.9275345110817126956e-4_f64) * t11992 + F::cast_from(0.9275345110817126956e-4_f64) * t11995 + F::cast_from(0.77294542590142724635e-6_f64) * t11998;
    t12568
}
