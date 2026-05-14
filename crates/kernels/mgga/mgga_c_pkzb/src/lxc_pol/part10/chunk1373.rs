//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1373/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1373<F: Float>(t2234: F, t6199: F, t9867: F, t18589: F, t18592: F, t2198: F, t3739: F, t10175: F, t2313: F, t898: F, t2340: F, t9762: F, t3135: F, t2317: F, t2320: F, t2332: F) -> (F, F, F, F, F, F, F) {
    let t27443 = 0.51726012919273400301e3 * t6199 * t9867 * t2234;
    let t27447 = 0.24955700379505800916e5 * t18589 * t3739 * t18592 * t2198;
    let t27450 = 0.11696447245269292414e1 * t898 * t10175 * t2313;
    let t27452 = 0.17315859105681463759e2 * t9762 * t2340;
    let t27453 = t3135 * t3135;
    let t27457 = 0.34631718211362927518e2 * t898 * t2317 * t27453 * t2320;
    let t27459 = 0.11696447245269292414e1 * t9762 * t2332;
    (t27443, t27447, t27450, t27452, t27453, t27457, t27459)
}
