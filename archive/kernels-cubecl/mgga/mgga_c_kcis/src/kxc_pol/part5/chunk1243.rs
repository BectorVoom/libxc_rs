//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1243/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1243<F: Float>(t19825: F, t19827: F, t19829: F, t19831: F, t19833: F, t19835: F, t19838: F, t19841: F, t19842: F, t19845: F, t20211: F, t187: F, t20813: F) -> F {
    let t20814 = -t19825 + t19827 + t19829 - t19831 + t19833 - t19835 + t19838 - t19841 + t19842 - t19845 + t20211;
    let t20817 = t19825 - t19827 - t19829 + t19831 - t19833 + t19835 - t19838 + t19841 - t19842 + t19845 - t20211 + t187 * (t20813 + t20814);
    t20817
}
