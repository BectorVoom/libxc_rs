//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1040/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1040<F: Float>(t24063: F, t24064: F, t3332: F, t37988: F, t37998: F, t38002: F, t39937: F, t39940: F, t39942: F, t39943: F, t39945: F, t39947: F, t39950: F, t39951: F, t10868: F, t7628: F, t7629: F) -> (F, F) {
    let t39954 = t24063 * t3332 * t24064;
    let t39956 = 0.5239643197851989015e-1 * t39937 - t39940 + t39942 + t39943 - 0.13869154784086829701e1 * t37988 - 0.43341108700271342816e-1 * t39945 + 0.2600466522016280569e0 * t39947 + 0.97574405393827830186e-2 * t37998 - t38002 + t39950 + 0.43341108700271342816e-1 * t39951 + 0.13099107994629972538e-1 * t39954;
    let t39958 = t7628 * t10868 * t7629;
    (t39956, t39958)
}
