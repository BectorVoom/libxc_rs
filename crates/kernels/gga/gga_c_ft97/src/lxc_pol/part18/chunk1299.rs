//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1299/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1299<F: Float>(t22638: F, t26612: F, t12374: F, t5828: F, t100601: F, t100634: F, t1008: F, t100915: F, t1013: F, t104941: F, t104945: F, t18: F, t2030: F, t2071: F, t23701: F, t23705: F, t23732: F, t23774: F, t23825: F, t23842: F, t26692: F, t26716: F, t39847: F, t423: F, t554: F, t5579: F, t61637: F, t72: F, t8833: F, t94620: F, t94622: F, t94626: F, t94640: F) -> (F,) {
    let t104988 = t22638 * t26612;
    let t105007 = t12374 * t5828;
    let t105010 = 0.74086667880658436219e-2 * t94620 + 0.11113000182098765433e-1 * t94622 + 0.29634667152263374487e-1 * t94626 + 0.13592055123908617004e1 * t39847 * t104941 + 0.48327307107230638237e1 * t8833 * t104945 + 0.13335600218518518519e0 * t23705 * t100634 * t423 * t18 * t554 - 0.1611184118048991131e0 * t23701 * t100915 - 0.10001700163888888889e0 * t26692 * t100601 - 0.33339000546296296298e-1 * t94640 + 0.28195722065857344792e1 * t23842 * t104988 - 0.28195722065857344792e1 * t23825 * t104988 + 0.20003400327777777778e0 * t23732 * t5579 * t72 * t1013 * t2030 + 0.20003400327777777778e0 * t23732 * t5579 * t72 * t1008 * t2071 - 0.60010200983333333334e0 * t23774 * t5579 * t72 * t61637 + 0.40006800655555555556e0 * t105007 * t26716;
    (t105010,)
}
