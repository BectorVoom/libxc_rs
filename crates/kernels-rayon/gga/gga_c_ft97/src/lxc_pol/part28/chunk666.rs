//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 666/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk666(t25749: f64, t5838: f64, t1008: f64, t53: f64, t72: f64, t5591: f64, t1013: f64, t538: f64, t16762: f64, t1014: f64, t2036: f64, t2043: f64, t23732: f64, t23774: f64, t23825: f64, t23839: f64, t23866: f64, t25746: f64, t25799: f64, t26601: f64, t26604: f64, t26608: f64, t26613: f64, t26617: f64, t26621: f64, t5579: f64, t5790: f64, t5813: f64, t6605: f64, t8812: f64) -> (f64, f64, f64, f64, f64) {
    let t26631 = t5838 * t25749;
    let t26634 = t72 * t1008 * t53;
    let t26635 = t5591 * t26634;
    let t26638 = t1013 * t538;
    let t26639 = t72 * t26638;
    let t26643 = t72 * t16762;
    let t26647 = -0.54738951849294959987e0_f64 * t8812 * t26601 - 0.10001700163888888889e0_f64 * t26604 * t6605 - 0.10001700163888888889e0_f64 * t5813 * t26608 + 0.24167761770734866964e0_f64 * t23825 * t26613 + 0.21895580739717983994e1_f64 * t23866 * t26617 - 0.22653425206514361674e0_f64 * t2043 * t26621 + 0.27369475924647479994e0_f64 * t2036 * t5790 * t1014 - 0.33339000546296296298e-1_f64 * t5838 * t25799 + 0.44452000728395061731e-1_f64 * t5838 * t25746 - 0.55565000910493827163e-2_f64 * t26631 - 0.24167761770734866964e0_f64 * t23839 * t26635 + 0.20003400327777777778e0_f64 * t23732 * t5579 * t26639 - 0.30005100491666666667e0_f64 * t23774 * t5579 * t26643;
    (t26635, t26638, t26639, t26643, t26647)
}
