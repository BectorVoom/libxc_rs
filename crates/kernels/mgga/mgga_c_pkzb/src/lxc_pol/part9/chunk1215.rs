//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1215/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1215<F: Float>(t1088: F, t5870: F, t1100: F, t5490: F, t663: F, t7502: F, t685: F, t1096: F, t1108: F, t17385: F, t17605: F, t17707: F, t1933: F, t1941: F, t1950: F, t21173: F, t21179: F, t21186: F, t21196: F, t2796: F, t2816: F, t2849: F, t5820: F, t5846: F, t5866: F, t5874: F, t5877: F, t5900: F, t702: F, t7324: F, t7447: F, t7475: F) -> (F, F) {
    let t21203 = t1088 * t5870;
    let t21212 = t1100 * t5490;
    let t21215 = t7502 * t663;
    let t21217 = F::new(3.0) * t21215 * t685;
    let t21218 = F::new(0.6207121550312808036e4) * t21173 * t17707 * t702 + F::new(3.0) * t7447 * t1933 + F::new(0.96491876992155210402e2) * t21179 * t1941 + F::new(0.17544670867903938621e1) * t1950 * t7475 - t21186 + t21196 + F::new(0.5848223622634646207e0) * t17385 * t1108 + F::new(0.17544670867903938621e1) * t5877 * t2849 + F::new(1.0) * t2796 * t5866 + F::new(0.2069040516770936012e4) * t21203 * t5874 + F::new(1.0) * t17605 * t1096 + F::new(3.0) * t5820 * t2816 + F::new(6.0) * t7324 * t5900 + F::new(0.10254018858216406658e4) * t21212 * t5846 - t21217;
    (t21217, t21218)
}
