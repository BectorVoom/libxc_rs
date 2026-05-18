//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1288/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1288<F: Float>(t1039: F, t12227: F, t354: F, t44926: F, t44928: F, t44931: F, t44933: F, t44935: F, t44937: F, t44940: F, t44942: F, t44953: F, t44957: F, t44962: F, t44964: F, t44971: F, t44979: F, t44986: F, t44988: F, t44997: F, t45006: F, t45011: F, t45015: F, t45023: F, t45026: F, t45030: F, t45034: F, t45036: F, t45040: F, t45044: F, t45054: F, t45079: F, t45101: F) -> F {
    let t45109 = -t44926 - t44928 - t44931 - t44933 + t44935 - t44937 + t44940 - t44942 + t354 * (t44953 + t44957 + t44962 + t44964 + t44971 + t44979 + t44986 + t44988 + t44997 + t45006 + t45011 + t45015 + t45040 + t45054 + t45079 + t45101) - t45023 - t45026 + t45030 + F::new(2.0) * t1039 * t12227 + t45034 + t45036 + t45044;
    t45109
}
