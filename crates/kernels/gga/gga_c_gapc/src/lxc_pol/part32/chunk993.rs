//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 993/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk993<F: Float>(t1086: F, t7739: F, t11990: F, t2597: F, t7503: F, t11320: F, t325: F, t11938: F, t11928: F, t11931: F, t11935: F, t11939: F, t11942: F, t11946: F, t11949: F, t11951: F, t11955: F, t11958: F, t11962: F, t11967: F, t11972: F, t11975: F, t11981: F, t11984: F, t11988: F) -> (F, F, F, F) {
    let t11991 = t1086 * t7739;
    let t11992 = t11990 * t11991;
    let t11994 = t2597 * t7503;
    let t11995 = t11990 * t11994;
    let t11997 = t325 * t11320;
    let t11998 = t11997 * t11938;
    let t12000 = F::new(0.24170290702783640635e-8) * t11928 + F::new(0.33764099580923002116e-6) * t11931 - F::new(0.3373480902777777778e-5) * t11935 - F::new(0.343574241813184411e-6) * t11939 + F::new(0.56273499301538336859e-8) * t11942 - F::new(0.11049275749843950005e-8) * t11946 - F::new(0.16573913624765925007e-7) * t11949 - F::new(0.33147827249531850014e-7) * t11951 - F::new(0.5060221354166666667e-4) * t11955 + F::new(0.10120442708333333334e-4) * t11958 + F::new(0.21084255642361111112e-6) * t11962 - F::new(0.18448723687065972223e-6) * t11967 - F::new(0.23671453668231209419e-4) * t11972 + F::new(0.28985453471303521737e-5) * t11975 - F::new(0.17924088107215033639e-9) * t11981 + F::new(0.16573913624765925007e-7) * t11984 + F::new(0.17376185052903442709e-3) * t11988 - F::new(0.2318836277704281739e-4) * t11992 + F::new(0.2318836277704281739e-4) * t11995 + F::new(0.19323635647535681159e-6) * t11998;
    (t11991, t11994, t11997, t12000)
}
