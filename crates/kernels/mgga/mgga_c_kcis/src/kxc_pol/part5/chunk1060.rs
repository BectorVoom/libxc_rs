//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1060/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1060<F: Float>(t1962: F, t4016: F, t1014: F, t5872: F, t1928: F, t4161: F, t2820: F, t5659: F, t86: F, t5664: F, t11913: F, t5656: F) -> (F, F, F, F, F, F, F) {
    let t17250 = t4016 * t1962;
    let t17259 = t1014 * t5872;
    let t17260 = F::new(0.33163888888888888888e-2) * t17259;
    let t17261 = t4161 * t1928;
    let t17266 = t86 * t2820 * t5659;
    let t17267 = t17266 * t5664;
    let t17268 = F::new(0.3684876543209876543e-2) * t17267;
    let t17274 = t11913 * t5656;
    (t17250, t17259, t17260, t17261, t17267, t17268, t17274)
}
