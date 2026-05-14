//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 864/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk864<F: Float>(t1905: F, t23327: F, t1651: F, t5630: F, t1902: F, t1643: F, t8518: F, t1307: F, t487: F) -> (F, F, F, F, F, F) {
    let t23328 = t23327 * t1905;
    let t23331 = t5630 * t1651;
    let t23332 = t1902 * t23331;
    let t23335 = t5630 * t1643;
    let t23336 = t8518 * t23335;
    let t23339 = t487 * t1307;
    (t23328, t23331, t23332, t23335, t23336, t23339)
}
