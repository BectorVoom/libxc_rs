//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1135/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1135<F: Float>(t762: F, t9707: F, t2567: F, t6061: F, t27889: F, t761: F, t107996: F, t107998: F, t108000: F, t108002: F, t108006: F, t108010: F, t108014: F, t108018: F, t108022: F, t108026: F, t108030: F) -> (F, F, F, F) {
    let t110010 = t9707 * t762;
    let t110019 = t2567 * t6061;
    let t110024 = t761 * t27889;
    let t110041 = 2.0 / 27.0 * t107996;
    let t110042 = 2.0 / 27.0 * t107998;
    let t110043 = 2.0 / 81.0 * t108000;
    let t110044 = t108002 / 54.0;
    let t110052 = t110041 + t110042 - t110043 + t110044 - 2.0 / 9.0 * t108006 - t108010 / 9.0 + 4.0 / 9.0 * t108014 - 2.0 / 9.0 * t108018 - t108022 / 9.0 - 2.0 / 9.0 * t108026 + 4.0 / 9.0 * t108030;
    (t110010, t110019, t110024, t110052)
}
