//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1125/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1125(t11784: f64, t11983: f64, t3784: f64, t3788: f64, t7241: f64, t11990: f64, t19196: f64, t2597: f64, t1086: f64, t11790: f64, t22581: f64, t33928: f64, t33930: f64, t33932: f64, t33935: f64, t33937: f64, t33939: f64, t33941: f64) -> f64 {
    let t33943 = t11784 * t11983;
    let t33946 = t3784 * t7241 * t3788;
    let t33949 = t11990 * t2597 * t19196;
    let t33952 = t11790 * t1086 * t22581;
    let t33954 = 0.11254699860307667372e-7_f64 * t33928 - 0.50595483470764842601e-7_f64 * t33930 + 0.56863432614185654571e-5_f64 * t33932 + 0.12650553385416666667e-5_f64 * t33935 + 0.11254699860307667372e-7_f64 * t33937 + 0.11254699860307667372e-6_f64 * t33939 + 0.16573913624765925007e-7_f64 * t33941 + 0.33147827249531850014e-7_f64 * t33943 + 0.16573913624765925007e-7_f64 * t33946 - 0.6956508833112845217e-4_f64 * t33949 - 0.34752370105806885418e-3_f64 * t33952;
    t33954
}
