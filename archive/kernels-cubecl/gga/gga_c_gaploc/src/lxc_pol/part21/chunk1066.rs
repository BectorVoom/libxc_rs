//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1066/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1066<F: Float>(t10007: F, t7068: F, t10012: F, t1984: F, t9804: F, t5501: F, t935: F, t2530: F, t321: F, t5580: F, t7802: F, t7809: F) -> (F, F, F, F, F, F, F) {
    let t22980 = t10007 * t7068;
    let t22984 = t10012 * t7068;
    let t23000 = t1984 * t9804;
    let t23021 = t5501 * t935;
    let t23092 = t321 * t2530;
    let t23099 = t5580 * t7802;
    let t23104 = t5580 * t7809;
    (t22980, t22984, t23000, t23021, t23092, t23099, t23104)
}
