//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1185/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1185<F: Float>(t27: F, t799: F, t89: F, t90304: F, t5225: F, t43524: F, t5299: F, t2681: F, t71238: F, t83718: F, t83720: F, t83722: F, t89865: F, t89868: F, t89872: F, t89875: F, t89879: F, t89883: F, t89887: F, t89891: F) -> (F, F, F, F, F, F) {
    let t90307 = t89 * t27 * t799 * t90304;
    let t90308 = t5225 * t5225;
    let t90311 = t89 * t27 * t43524 * t90308;
    let t90313 = t5299 * t5299;
    let t90316 = t89 * t27 * t2681 * t90313;
    let t90322 = F::new(40.0) / F::new(9.0) * t89865 - F::new(12.0) * t89868 + F::new(40.0) / F::new(27.0) * t89872 - F::new(20.0) / F::new(9.0) * t89875 + F::new(4.0) / F::new(3.0) * t89879 - F::new(8.0) / F::new(3.0) * t89883 - F::new(4.0) * t89887 - F::new(16.0) / F::new(3.0) * t89891 - t90307 + F::new(24.0) * t90311 + F::new(6.0) * t90316 - F::new(4.0) / F::new(3.0) * t83718 - F::new(4.0) / F::new(3.0) * t83720 + F::new(8.0) / F::new(9.0) * t83722 + F::new(16.0) / F::new(3.0) * t71238;
    (t90307, t90308, t90311, t90313, t90316, t90322)
}
