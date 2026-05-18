//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 954/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk954<F: Float>(t10373: F, t13625: F, t13629: F, t13633: F, t13635: F, t13637: F, t13639: F, t13643: F, t13645: F, t13648: F, t9645: F, t14798: F) -> F {
    let t14809 = -F::new(0.13335600218518518519e0) * t13625 - F::new(0.11113000182098765433e-1) * t9645 + F::new(0.77791001274691358028e-1) * t13629 - F::new(0.33339000546296296298e-1) * t13633 - F::new(0.29634667152263374486e-1) * t13635 - F::new(0.4445200072839506173e-1) * t13637 - F::new(0.59269334304526748973e-1) * t13639 + F::new(0.29634667152263374487e-1) * t13643 + t10373 + F::new(0.8890400145679012346e-1) * t13645 - F::new(0.37043333940329218109e-2) * t13648;
    let t14810 = t14798 + t14809;
    t14810
}
