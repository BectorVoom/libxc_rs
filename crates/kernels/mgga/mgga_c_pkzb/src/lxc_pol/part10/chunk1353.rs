//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1353/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1353<F: Float>(t10066: F, t3206: F, t6475: F, t10195: F, t178: F, t915: F, t10050: F, t2380: F, t22445: F, t22452: F, t22461: F, t22469: F, t22474: F, t22933: F, t22936: F, t22944: F, t22947: F, t22950: F, t22988: F, t2384: F) -> (F,) {
    let t26970 = t3206 * t6475 * t10066;
    let t26975 = t915 * t10195 * t178;
    let t26981 = t2380 * t6475 * t10050;
    let t26984 = -0.11433071498151929859e-2 * t22445 - 0.57165357490759649296e-3 * t22452 - 5.0 / 243.0 * t22461 - 0.11433071498151929859e-2 * t22469 + 0.3811023832717309953e-3 * t22474 - 0.28582678745379824648e-3 * t22933 + 0.57165357490759649296e-3 * t26970 - 0.11433071498151929859e-2 * t22936 + 0.19055119163586549765e-3 * t22944 - 0.28963781128651555642e-1 * t26975 * t2384 - 0.57165357490759649296e-3 * t22947 + 0.3811023832717309953e-3 * t22950 - 0.11433071498151929859e-2 * t26981 - 0.3811023832717309953e-3 * t22988;
    (t26984,)
}
