//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1104/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1104<F: Float>(t40842: F, t40844: F, t40846: F, t40850: F, t40852: F, t40854: F, t40856: F, t40858: F, t40860: F, t40862: F, t40866: F, t40868: F, t40870: F) -> F {
    let t44057 = F::cast_from(0.5987120850931904282e-1_f64) * t40842 + F::cast_from(0.16364796992547205038e0_f64) * t40844 + F::cast_from(0.8182398496273602519e-1_f64) * t40846 - F::cast_from(0.2727466165424534173e0_f64) * t40850 - F::cast_from(0.13637330827122670865e0_f64) * t40852 - F::cast_from(0.5454932330849068346e-1_f64) * t40854 - F::cast_from(0.2727466165424534173e-1_f64) * t40856 + F::cast_from(0.40911992481368012596e-1_f64) * t40858 + F::cast_from(0.20455996240684006298e-1_f64) * t40860 - F::cast_from(0.5454932330849068346e-1_f64) * t40862 - F::cast_from(0.2727466165424534173e-1_f64) * t40866 - F::cast_from(0.2727466165424534173e-1_f64) * t40868 - F::cast_from(0.13637330827122670865e-1_f64) * t40870;
    t44057
}
