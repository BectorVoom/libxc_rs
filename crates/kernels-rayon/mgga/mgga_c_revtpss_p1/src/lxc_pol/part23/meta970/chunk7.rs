//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3277/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3277(t13999: f64, t22833: f64, t13944: f64, t21990: f64, t22046: f64, t22893: f64, t3934: f64, t46879: f64, t46885: f64, t46889: f64, t46947: f64, t47199: f64, t48881: f64, t48905: f64, t48909: f64, t48947: f64, t48982: f64, t5671: f64, t5673: f64, t74505: f64, t74507: f64, t74511: f64, t74522: f64, t74547: f64, t9955: f64) -> f64 {
    let t86156 = t13999 * t22833;
    let t86162 = 0.36014175219178579057e0_f64 * t74505 - 0.12004725073059526352e0_f64 * t74507 + 0.45732285992607719436e-3_f64 * t48881 + 0.16262400898971305032e-2_f64 * t74511 + 0.28900264064772933811e-2_f64 * t46879 + t46885 + 0.45178982497454656792e-6_f64 * t46889 + 0.76230004213927992336e-5_f64 * t74522 + t48905 - 0.24098469264142313933e-5_f64 * t48909 - 0.45738002528356795401e-4_f64 * t46947 + 0.45351183609335988442e0_f64 * t48947 + 7.0_f64 / 48.0_f64 * t74547 + 0.38586616306262763276e-2_f64 * t5671 * t5673 * t22046 * t21990 - 0.12846167376791569079e-2_f64 * t47199 - 0.81312004494856525158e-3_f64 * t48982 - 0.60023625365297631763e-2_f64 * t86156 - 0.12862205435420921092e-1_f64 * t3934 * t9955 * t13944 * t22893;
    t86162
}
