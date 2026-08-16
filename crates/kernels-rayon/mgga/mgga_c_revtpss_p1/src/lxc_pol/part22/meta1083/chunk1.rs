//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3914/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3914(t1398: f64, t2782: f64, t4086: f64, t543: f64, t6888: f64, t75198: f64, t3999: f64, t13921: f64, t1883: f64, t21981: f64, t3924: f64, t4004: f64, t47395: f64, t49268: f64, t49308: f64, t49313: f64, t49321: f64, t49325: f64, t49346: f64, t5675: f64, t5735: f64, t5745: f64, t5755: f64, t820: f64) -> f64 {
    let t75215 = t2782 * t4086 * t6888 * t1398 * t543;
    let t75219 = t2782 * t4086 * t75198 * t543;
    let t75228 = t3999 * t6888;
    let t75242 = 0.10975748638225852664e-1_f64 * t75215 + 0.21951497276451705328e-1_f64 * t75219 + 0.10975748638225852664e-1_f64 * t49308 - t47395 + 0.10975748638225852664e-1_f64 * t49313 - 0.13170898365871023197e1_f64 * t5755 * t5735 * t13921 + 0.52039682876708176102e-1_f64 * t49321 - 0.39029762157531132076e-1_f64 * t49325 + 0.13170898365871023197e1_f64 * t820 * t75228 * t4004 - 0.13170898365871023197e1_f64 * t5755 * t21981 * t3924 - 0.13170898365871023197e1_f64 * t820 * t49268 * t1883 + 0.21951497276451705328e-1_f64 * t49346 + 0.52683593463484092788e1_f64 * t5745 * t75198 * t5675;
    t75242
}
