//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3914/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3914<F: Float>(t1398: F, t2782: F, t4086: F, t543: F, t6888: F, t75198: F, t3999: F, t13921: F, t1883: F, t21981: F, t3924: F, t4004: F, t47395: F, t49268: F, t49308: F, t49313: F, t49321: F, t49325: F, t49346: F, t5675: F, t5735: F, t5745: F, t5755: F, t820: F) -> F {
    let t75215 = t2782 * t4086 * t6888 * t1398 * t543;
    let t75219 = t2782 * t4086 * t75198 * t543;
    let t75228 = t3999 * t6888;
    let t75242 = F::cast_from(0.10975748638225852664e-1_f64) * t75215 + F::cast_from(0.21951497276451705328e-1_f64) * t75219 + F::cast_from(0.10975748638225852664e-1_f64) * t49308 - t47395 + F::cast_from(0.10975748638225852664e-1_f64) * t49313 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t5735 * t13921 + F::cast_from(0.52039682876708176102e-1_f64) * t49321 - F::cast_from(0.39029762157531132076e-1_f64) * t49325 + F::cast_from(0.13170898365871023197e1_f64) * t820 * t75228 * t4004 - F::cast_from(0.13170898365871023197e1_f64) * t5755 * t21981 * t3924 - F::cast_from(0.13170898365871023197e1_f64) * t820 * t49268 * t1883 + F::cast_from(0.21951497276451705328e-1_f64) * t49346 + F::cast_from(0.52683593463484092788e1_f64) * t5745 * t75198 * t5675;
    t75242
}
