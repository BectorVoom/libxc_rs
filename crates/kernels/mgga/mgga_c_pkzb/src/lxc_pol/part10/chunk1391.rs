//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1391/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1391<F: Float>(t237: F, t27550: F, t27573: F, t27609: F, t27666: F, t27706: F, t27801: F, t27846: F, t27944: F, t3162: F, t8028: F, t2336: F, t9762: F, t3841: F, t6117: F, t3147: F, t8296: F) -> (F, F, F, F, F) {
    let t27948 = t237 * (t27550 + t27573 + t27609 + t27666 + t27706 + t27801 + t27846 + t27944);
    let t27950 = 0.69263436422725855034e2 * t8028 * t3162;
    let t27952 = 0.5848223622634646207e0 * t9762 * t2336;
    let t27954 = 0.17315859105681463759e2 * t6117 * t3841;
    let t27956 = 0.23392894490538584828e1 * t3147 * t8296;
    (t27948, t27950, t27952, t27954, t27956)
}
