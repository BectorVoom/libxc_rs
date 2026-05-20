//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3277/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3277<F: Float>(t13999: F, t22833: F, t13944: F, t21990: F, t22046: F, t22893: F, t3934: F, t46879: F, t46885: F, t46889: F, t46947: F, t47199: F, t48881: F, t48905: F, t48909: F, t48947: F, t48982: F, t5671: F, t5673: F, t74505: F, t74507: F, t74511: F, t74522: F, t74547: F, t9955: F) -> F {
    let t86156 = t13999 * t22833;
    let t86162 = F::cast_from(0.36014175219178579057e0_f64) * t74505 - F::cast_from(0.12004725073059526352e0_f64) * t74507 + F::cast_from(0.45732285992607719436e-3_f64) * t48881 + F::cast_from(0.16262400898971305032e-2_f64) * t74511 + F::cast_from(0.28900264064772933811e-2_f64) * t46879 + t46885 + F::cast_from(0.45178982497454656792e-6_f64) * t46889 + F::cast_from(0.76230004213927992336e-5_f64) * t74522 + t48905 - F::cast_from(0.24098469264142313933e-5_f64) * t48909 - F::cast_from(0.45738002528356795401e-4_f64) * t46947 + F::cast_from(0.45351183609335988442e0_f64) * t48947 + F::new(7.0) / F::new(48.0) * t74547 + F::cast_from(0.38586616306262763276e-2_f64) * t5671 * t5673 * t22046 * t21990 - F::cast_from(0.12846167376791569079e-2_f64) * t47199 - F::cast_from(0.81312004494856525158e-3_f64) * t48982 - F::cast_from(0.60023625365297631763e-2_f64) * t86156 - F::cast_from(0.12862205435420921092e-1_f64) * t3934 * t9955 * t13944 * t22893;
    t86162
}
