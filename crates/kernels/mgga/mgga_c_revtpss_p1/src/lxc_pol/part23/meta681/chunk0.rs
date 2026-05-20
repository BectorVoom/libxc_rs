//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2422/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2422<F: Float>(t3566: F, t3781: F, t5330: F, t3362: F, t404: F, t43813: F, t1175: F, t12485: F, t1156: F, t12469: F, t3450: F, t3475: F, t426: F) -> (F, F, F, F, F, F) {
    let t44951 = t3566 * t3781;
    let t44952 = t44951 * t5330;
    let t44958 = F::new(1.0) / t404 / t3362;
    let t45000 = F::cast_from(0.18467901234567901234e0_f64) * t43813;
    let t45064 = t1175 * t12485;
    let t45080 = t1156 * t12469;
    let t45085 = t426 / t3475 / t3450;
    (t44952, t44958, t45000, t45064, t45080, t45085)
}
