//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1008/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1008(t10054: f64, t3040: f64, t3267: f64, t8556: f64, t13126: f64, t2194: f64, t13096: f64, t313: f64, t795: f64, t1445: f64, t2087: f64, t43240: f64) -> (f64, f64, f64, f64, f64) {
    let t44027 = 0.35750489951850426669e0_f64 * t10054 * t3040;
    let t44029 = 0.23833659967900284446e0_f64 * t3267 * t8556;
    let t44030 = t2194 * t13126;
    let t44033 = t313 * t795 * t13096;
    let t44038 = 0.62115540045351614476e2_f64 * t2087 * t1445 * t43240;
    (t44027, t44029, t44030, t44033, t44038)
}
