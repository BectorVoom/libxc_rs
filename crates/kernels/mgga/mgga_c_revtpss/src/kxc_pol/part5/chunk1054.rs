//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1054/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1054<F: Float>(t13985: F, t9816: F, t5706: F, t9962: F, t4000: F, t820: F, t844: F, t5677: F, t13847: F, t13848: F, t1399: F, t2713: F, t3964: F, t5617: F) -> (F, F, F, F, F) {
    let t13987 = F::new(0.10164000561857065645e-3) * t9816 * t13985;
    let t13988 = t9962 * t5706;
    let t13999 = t820 * t4000 * t844;
    let t14001 = F::new(0.40015750243531754508e-2) * t13999 * t5677;
    let t14005 = t13847 * t13848 * t1399;
    let t14007 = F::new(0.25410001404642664112e-4) * t9816 * t14005;
    let t14013 = t3964 * t2713 * t5617;
    (t13987, t13988, t14001, t14007, t14013)
}
