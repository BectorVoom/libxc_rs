//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 987/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk987<F: Float>(t2: F, t34918: F, t1969: F, t379: F, t5899: F, t23649: F, t34832: F, t23657: F, t27147: F, t32924: F, t9432: F, t23671: F, t34843: F, t139214: F, t139224: F, t26950: F, t32897: F) -> (F, F, F, F, F) {
    let t148306 = t2 * t34918;
    let t148309 = t5899 * t1969 * t148306 * t379;
    let t148311 = t23649 * t34832;
    let t148315 = t23657 * t9432 * t32924 * t27147;
    let t148319 = t23657 * t23671 * t34843 * t379;
    let t148323 = t32897 * t139224 * t139214 * t26950;
    (t148309, t148311, t148315, t148319, t148323)
}
