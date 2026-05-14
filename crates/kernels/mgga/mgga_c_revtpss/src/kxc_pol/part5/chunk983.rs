//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 983/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk983<F: Float>(t2689: F, t5618: F, t1413: F, t5591: F, t547: F, t807: F, t5609: F, t808: F, t9845: F, t1885: F, t9909: F, t1399: F, t1872: F, t9818: F, t9816: F, t5706: F, t9962: F) -> (F, F, F, F, F, F) {
    let t13949 = t2689 * t5618;
    let t13951 = t1413 * t5591;
    let t13952 = t547 * t13951;
    let t13954 = 0.57165357490759649296e-4 * t807 * t13952;
    let t13955 = t808 * t5609;
    let t13956 = t9845 * t13955;
    let t13959 = t9909 * t1885;
    let t13985 = t9818 * t1872 * t1399;
    let t13987 = 0.10164000561857065645e-3 * t9816 * t13985;
    let t13988 = t9962 * t5706;
    (t13949, t13954, t13956, t13959, t13987, t13988)
}
