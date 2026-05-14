//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 779/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk779<F: Float>(t3124: F, t406: F, t3142: F, t741: F, t750: F, t5433: F, t5437: F, t5441: F, t5444: F, t5451: F, t5454: F, t7751: F, t7753: F, t7756: F, t5459: F, t5463: F, t5467: F, t5474: F, t5479: F, t5846: F, t5847: F, t5853: F, t7776: F, t7781: F, t7785: F) -> (F, F) {
    let t8946 = t406 * t3124;
    let t8948 = t3142 * t741;
    let t8950 = t3142 * t750;
    let t8954 = t5433 - t5437 + t5441 - 4.0 * t8946 - 0.11696447245269292414e1 * t8948 + 0.17315859105681463759e2 * t8950 + t5444 - t7751 - 0.1143056e0 * t7753 + 0.16008171603946666666e-1 * t7756 + t5451 + t5454;
    let t8961 = -t5459 + t5463 + t5467 - 0.22581706311111111111e-2 * t7776 + t7781 + 0.84681398666666666665e-3 * t7785 + t5474 - t5479 - t5846 - 12.0 * t5847 + t5853;
    (t8954, t8961)
}
