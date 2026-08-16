//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3916/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3916(t1398: f64, t23037: f64, t10022: f64, t2782: f64, t22352: f64, t2435: f64, t13805: f64, t14193: f64, t21981: f64, t22005: f64, t22016: f64, t4057: f64, t47442: f64, t47444: f64, t49403: f64, t49407: f64, t49439: f64, t5675: f64, t5745: f64, t5755: f64, t73861: f64, t74922: f64, t74982: f64, t9840: f64) -> f64 {
    let t75267 = t23037 * t1398;
    let t75269 = t2782 * t10022 * t75267;
    let t75274 = t2435 * t22352;
    let t75295 = 0.39029762157531132076e-1_f64 * t49403 + t47442 + 0.19514881078765566038e-1_f64 * t49407 - 0.21951497276451705328e-1_f64 * t75269 - 0.13170898365871023197e1_f64 * t5755 * t21981 * t4057 + 0.73171657588172351096e-2_f64 * t75274 + 0.26341796731742046394e1_f64 * t5745 * t21981 * t9840 + 0.52683593463484092788e1_f64 * t5745 * t74922 * t5675 + 0.39512695097613069591e1_f64 * t5745 * t22005 * t9840 - 0.23707617058567841754e2_f64 * t14193 * t22005 * t13805 + 0.15805078039045227836e2_f64 * t49439 * t22005 * t73861 - 0.79025390195226139182e1_f64 * t14193 * t74982 * t22016 + 0.60712963356159538784e-1_f64 * t47444;
    t75295
}
