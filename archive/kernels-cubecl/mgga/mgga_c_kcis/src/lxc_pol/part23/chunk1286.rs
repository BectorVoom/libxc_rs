//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1286/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1286<F: Float>(t1610: F, t27614: F, t6176: F, t6188: F, t18210: F, t28815: F, t7968: F, t27563: F, t28714: F, t1889: F, t27615: F, t95024: F) -> (F, F, F, F, F) {
    let t99020 = t6176 * t27614 * t6188 * t1610;
    let t99023 = t18210 * t28815;
    let t99024 = t7968 * t99023;
    let t99035 = F::cast_from(0.23168402777777777778e-3_f64) * t28714 * t27563;
    let t99043 = t95024 * t1889 * t27615;
    (t99020, t99023, t99024, t99035, t99043)
}
