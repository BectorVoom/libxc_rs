//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3916/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3916<F: Float>(t1398: F, t23037: F, t10022: F, t2782: F, t22352: F, t2435: F, t13805: F, t14193: F, t21981: F, t22005: F, t22016: F, t4057: F, t47442: F, t47444: F, t49403: F, t49407: F, t49439: F, t5675: F, t5745: F, t5755: F, t73861: F, t74922: F, t74982: F, t9840: F) -> F {
    let t75267 = t23037 * t1398;
    let t75269 = t2782 * t10022 * t75267;
    let t75274 = t2435 * t22352;
    let t75295 = F::cast_from(0.39029762157531132076e-1_f64) * t49403 + t47442 + F::cast_from(0.19514881078765566038e-1_f64) * t49407 - F::cast_from(0.21951497276451705328e-1_f64) * t75269 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t21981 * t4057 + F::cast_from(0.73171657588172351096e-2_f64) * t75274 + F::cast_from(0.26341796731742046394e1_f64) * t5745 * t21981 * t9840 + F::cast_from(0.52683593463484092788e1_f64) * t5745 * t74922 * t5675 + F::cast_from(0.39512695097613069591e1_f64) * t5745 * t22005 * t9840 - F::cast_from(0.23707617058567841754e2_f64) * t14193 * t22005 * t13805 + F::cast_from(0.15805078039045227836e2_f64) * t49439 * t22005 * t73861 - F::cast_from(0.79025390195226139182e1_f64) * t14193 * t74982 * t22016 + F::cast_from(0.60712963356159538784e-1_f64) * t47444;
    t75295
}
