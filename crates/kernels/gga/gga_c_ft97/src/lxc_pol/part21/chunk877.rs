//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 877/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk877<F: Float>(t1701: F, t3404: F, t5546: F, t25749: F, t5838: F, t1008: F, t53: F, t72: F, t5591: F, t1013: F, t538: F, t16762: F, t1014: F, t2036: F, t2043: F, t23732: F, t23774: F, t23825: F, t23839: F, t23866: F, t25746: F, t25799: F, t26601: F, t26604: F, t26608: F, t26613: F, t26617: F, t5579: F, t5790: F, t5813: F, t6605: F, t8812: F) -> (F, F, F, F, F, F, F) {
    let t26621 = t1701 * t5546 * t3404;
    let t26631 = t5838 * t25749;
    let t26634 = t72 * t1008 * t53;
    let t26635 = t5591 * t26634;
    let t26638 = t1013 * t538;
    let t26639 = t72 * t26638;
    let t26643 = t72 * t16762;
    let t26647 = -0.54738951849294959987e0 * t8812 * t26601 - 0.10001700163888888889e0 * t26604 * t6605 - 0.10001700163888888889e0 * t5813 * t26608 + 0.24167761770734866964e0 * t23825 * t26613 + 0.21895580739717983994e1 * t23866 * t26617 - 0.22653425206514361674e0 * t2043 * t26621 + 0.27369475924647479994e0 * t2036 * t5790 * t1014 - 0.33339000546296296298e-1 * t5838 * t25799 + 0.44452000728395061731e-1 * t5838 * t25746 - 0.55565000910493827163e-2 * t26631 - 0.24167761770734866964e0 * t23839 * t26635 + 0.20003400327777777778e0 * t23732 * t5579 * t26639 - 0.30005100491666666667e0 * t23774 * t5579 * t26643;
    (t26621, t26631, t26634, t26635, t26639, t26643, t26647)
}
